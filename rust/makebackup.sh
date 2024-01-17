cd ..
git archive --format=zip -o "backups/bayes-star_$(git rev-parse --abbrev-ref HEAD)_$(date "+%Y-%b-%d-%H-%M").zip" HEAD


