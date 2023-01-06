#!/usr/bin/env bash

set -exuo pipefail

function validate_gh_login() {
  if ! gh auth status; then
    echo 'You are not logged into the GitHub CLI. You will need to login before running this script. Run `gh auth login --web` or set the environment variable GITHUB_TOKEN before proceding.'
  fi
}

function validate_tooling() {
  if ! which gh; then
    echo 'To run this script, you must have gh installed.'
    exit 1
  fi
}

function list_repo_workflows() {
  # TODO: Does not handle pagination.
  local -r OWNER="$1"
  local -r REPO="$2"
  gh api \
    -H "Accept: application/vnd.github+json" \
    "/repos/${OWNER}/${REPO}/actions/workflows"
}

function get_workflow() {
  local -r OWNER="$1"
  local -r REPO="$2"
  local -r WORKFLOW_ID="$3"
  gh api \
    -H "Accept: application/vnd.github+json" \
    "/repos/${OWNER}/${REPO}/actions/workflows/${WORKFLOW_ID}"
}

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

function list_runs() {
  local -r OWNER="$1"
  local -r REPO="$2"
  local -r TEMPLATE=$(cat <<'EOF' 
{{range .workflow_runs}}
  {{ 
    tablerow
    ( .conclusion | autocolor "green")
    ( .status | autocolor "yellow")    
    ( .run_started_at | autocolor "white")    
  }}
{{end}}
EOF)

  gh api \
    --header "Accept: application/vnd.github+json" \
    --method GET \
    --field "status=failure" \
    --template "${TEMPLATE}" \
    "/repos/${OWNER}/${REPO}/actions/runs"  
    #--jq '.workflow_runs[] | keys' \
}

# TODO: Use the --paginate flag to sequentially request all pages of data.

# TODO: To see how to build output templates, run `gh help formatting`
#       Of note: 
#       • autocolor
#       • pluck
#       • tablerow
#       • tablerender
#       • timefmt

function main() {
  validate_tooling
  validate_gh_login
  list_repo_workflows pulumi pulumi
  get_workflow pulumi pulumi 35076803
  get_workflow_usage pulumi pulumi 35076803
  list_runs pulumi pulumi
}

main