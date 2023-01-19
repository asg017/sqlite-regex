SHELL=/bin/bash

dates.png: ./dates/results.json Makefile
	observable-prerender @asg017/hyperfine-benchmark-viz1 chart  \
	  --redefine=width:900 \
	  --width 900 \
	  --redefine-file=data:json:$< \
	  --redefine-file=highlight:json:<(echo '["./sqlite-regex.sh"]') \
	  '--redefine=compareTo:./sqlite-regex.sh' \
	  '--redefine=title:Regex match "YYYY-MM-DD" dates in a 200k row, 8MB table.' \
	  '--redefine=footer:sqlite-regex version 0.0.1, ran on a 8GB RAM Macbook' \
	  -o $@