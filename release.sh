git checkout main
git add .
git commit -m "Minor changes in github workflow"
git push origin refs/heads/main

git checkout release
git merge refs/heads/main
git push origin refs/heads/release

git checkout main
