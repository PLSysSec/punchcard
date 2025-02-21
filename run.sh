#!/usr/bin/env bash

# Error when a non-zero exit code is received,
# echo commands before execution.
set -exuo pipefail

readonly REPO_LIST="repo-list.csv"

# Ensure users have a valid GitHub access token available
# for API requests.
function validate_gh_login() {
  if ! gh auth status; then
    echo 'You are not logged into the GitHub CLI. You will need to login before running this script. Run `gh auth login --web` or set the environment variable GITHUB_TOKEN before proceding.'
  fi
}

# Ensure users have the `gh` CLI tool installed.
function validate_tooling() {
  if ! which gh; then
    echo 'To run this script, you must have gh installed.'
    exit 1
  fi
}

# List all workflows for a repository.
# NB: Note yet paginated.
# @param OWNER: the username or org name for the repository.
# @param REPO: the name of the repository
function list_repo_workflows() {
  # TODO: Does not handle pagination.
  local -r OWNER="$1"
  local -r REPO="$2"
  gh api \
    -H "Accept: application/vnd.github+json" \
    "/repos/${OWNER}/${REPO}/actions/workflows"
}

# List the properties of a particular workflow.
# @param OWNER: the username or org name for the repository.
# @param REPO: the name of the repository
# @param WORKFLOW_ID: the ID of the workflow. Get this value 
# from list_repo_workflows.
function get_workflow() {
  local -r OWNER="$1"
  local -r REPO="$2"
  local -r WORKFLOW_ID="$3"
  gh api \
    -H "Accept: application/vnd.github+json" \
    "/repos/${OWNER}/${REPO}/actions/workflows/${WORKFLOW_ID}"
}

# Show the amount of time during the current billing cycle
# that is billable for GitHub Actions.
# @param OWNER: the username or org name for the repository.
# @param REPO: the name of the repository
# NB: This API call returns the number of billable minutes used by this
# workflow during the current billing cycle.
# This only applies to private repositories using hosted runners.
# See: https://docs.github.com/en/rest/actions/workflows?apiVersion=2022-11-28#get-workflow-usage
function get_workflow_usage() {
  local -r OWNER="$1"
  local -r REPO="$2"
  local -r WORKFLOW_ID="$3"
  gh api \
    -H "Accept: application/vnd.github+json" \
    "/repos/${OWNER}/${REPO}/actions/workflows/${WORKFLOW_ID}/timing"
}

# Show the recent failed workflow runs on this repository.
# @param OWNER: the username or org name for the repository.
# @param REPO: the name of the repository
function list_failed_runs() {
  local -r OWNER="$1"
  local -r REPO="$2"
  local -r TEMPLATE=$(cat "list_runs.tmpl")

  gh api \
    --header "Accept: application/vnd.github+json" \
    --method GET \
    --field "status=failure" \
    --template "${TEMPLATE}" \
    "/repos/${OWNER}/${REPO}/actions/runs"  
}

function list_all_runs() {
  local -r OWNER="$1"
  local -r REPO="$2"
  local -r TEMPLATE=$(cat "list_runs.tmpl")

  gh api \
    --header "Accept: application/vnd.github+json" \
    --method GET \
    --template "${TEMPLATE}" \
    "/repos/${OWNER}/${REPO}/actions/runs"  
}

function list_runs_with_startup_failure() {
  local -r OWNER="$1"
  local -r REPO="$2"
  local -r TEMPLATE=$(cat "list_runs.tmpl")
  local -r CSV_FILE="data/${OWNER}--${REPO}.csv"
  gh api \
    --header "Accept: application/vnd.github+json" \
    --method GET \
    --paginate \
    --field "status=startup_failure" \
    --template "${TEMPLATE}" \
    "/repos/${OWNER}/${REPO}/actions/runs" > "${CSV_FILE}"
    # Count the number of pushes with startup failure.
    wc -l "${CSV_FILE}"
}

function list_failures_no_paginate() {
  local -r OWNER="$1"
  local -r REPO="$2"
  local -r TEMPLATE=$(cat "list_runs.tmpl")
  local -r CSV_FILE="data/${OWNER}--${REPO}.csv"
  gh api \
    --header "Accept: application/vnd.github+json" \
    --method GET \
    --field "status=startup_failure" \
    --template "${TEMPLATE}" \
    "/repos/${OWNER}/${REPO}/actions/runs" > "${CSV_FILE}"
    # Count the number of pushes with startup failure.
    wc -l "${CSV_FILE}"
}

# The function reads the file described in REPO_LIST which is a list
# of rows. It splits each row on whitespace, accepting the first item
# as the owner and the second item as the repo name.
function fetch_failures_from_file() {
  # Read the file line by line.
  while read LINE; do
    failures_from_line "${LINE}"
  done < "${REPO_LIST}"  
}

function failures_from_line() {
  # Split the line on whitespace, grabbing the first lexeme as
  # the owner, and the second lexeme as the repo.
  local -r LINE="$1"
  local -r SPLIT=(${LINE})
  local -r OWNER="${SPLIT[0]}"
  local -r REPO="${SPLIT[1]}"
  list_runs_with_startup_failure "${OWNER}" "${REPO}"
}

# NB: Use the --paginate flag to sequentially request all pages of data.

# NB: As a feature of gh, you can filter data with jq. This is useful
#     when building a template, since it allows you to introspect on
#     the fields in the resulting JSON object. 
# e.g.: --jq '.workflow_runs[] | keys' \

# NB: To see how to build output templates, run `gh help formatting`
#       Of note: 
#       • autocolor
#       • pluck
#       • tablerow
#       • tablerender
#       • timefmt

# NB: GitHub Actions have a log retention period of 90 days max
# for public repositories.


# These functions appeared useful at first, but now there's not
# directly needed. This function exercises these API routes.
function test_api_helpers() {
  echo "Testing API calls."
  list_repo_workflows pulumi pulumi
  get_workflow pulumi pulumi 35076803
  get_workflow_usage pulumi pulumi 35076803
  list_failed_runs pulumi pulumi
  list_all_runs plSysSec punchcard
}

function main() {
  validate_tooling
  validate_gh_login
  echo "Your tooling has been validated. Starting API calls."
  fetch_failures_from_file
  # Buggy: receiving a 502 error
  # list_runs_with_startup_failure angular angular
  # list_runs_with_startup_failure nodejs node
  # list_failures_no_paginate facebook react-native
  # list_failures_no_paginate flutter flutter
  # list_failures_no_paginate elastic elasticsearch
  # list_runs_with_startup_failure mrdoob three.js
  # list_runs_with_startup_failure ant-design ant-design
  # list_runs_with_startup_failure microsoft vscode
}

main