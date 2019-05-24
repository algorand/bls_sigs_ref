SAGEFILES := $(basename $(notdir $(wildcard *.sage)))
PYFILES := $(addprefix __sage__, $(addsuffix .py,$(SAGEFILES)))
.PRECIOUS: $(PYFILES)

TEST_INPUTS ?= rfc6979
DEBUG ?= 0
ifeq ($(DEBUG),1)
	DFLAG := -d
else
	DFLAG :=
endif

.PHONY: pyfiles
pyfiles: $(PYFILES)

__sage__%.py: %.sage
	@echo "Parsing $<"
	@sage --preparse $<
	@mv $<.py $@

sig_%: bls_sig_%.sage pyfiles
	sage $< -T ../test-vectors/$(TEST_INPUTS) $(DFLAG) | tee $<_$(TEST_INPUTS)_$(DEBUG).log

hash_%: opt_sswu_%.sage pyfiles
	sage $< -T ../test-vectors/$(TEST_INPUTS) $(DFLAG) | tee $<_$(TEST_INPUTS)_$(DEBUG).log

clean:
	rm -f __sage__*.py *.pyc *.sage.py *.log
