.PHONY: 2015 2016 2017 2018 2019 2022 2023

2015:
	@bash ./run.sh $(MAKECMDGOALS)

2016:
	@bash ./run.sh $(MAKECMDGOALS)

2017:
	@bash ./run.sh $(MAKECMDGOALS)

2018:
	@bash ./run.sh $(MAKECMDGOALS)

2019:
	@bash ./run.sh $(MAKECMDGOALS)

2022:
	@bash ./run.sh $(MAKECMDGOALS)

2023:
	@bash ./run.sh $(MAKECMDGOALS)
	
%:
	@true
