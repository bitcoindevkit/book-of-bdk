# This script will build the website and push it to the `prod` branch of the repo,
# publishing it automatically to https://bookofbdk.com.

set -euo pipefail

rm -rf ./site/
mkdocs build
cd site
git init .
git switch --create prod
git add .
git commit --message "Deploy $(date +"%Y-%m-%d")"
git remote add upstream git@github.com:bitcoindevkit/book-of-bdk.git
git push upstream prod --force
cd ..
