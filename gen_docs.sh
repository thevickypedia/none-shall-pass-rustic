rm -rf docs
mkdir docs
touch docs/.nojekyll
curl -JLo markdown.zip http://daringfireball.net/projects/downloads/Markdown_1.0.1.zip
unzip markdown.zip
cd Markdown_1.0.1 && mv Markdown.pl ../ && cd .. && rm -rf Markdown_1.0.1 markdown.zip
perl Markdown.pl --html4tags README.md > docs/index.html
rm Markdown.pl
cargo clippy --no-deps --fix --allow-dirty

# to convert more than one markdown file to html
# for i in ./*.md; do perl markdown.pl --html4tags $i > docs/${i%.*}.html; done;
