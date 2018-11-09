all:
	xelatex slides.tex
	pygmentex slides.snippets
	xelatex slides.tex
