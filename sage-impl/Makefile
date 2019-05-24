SAGEFILES := bls_sig_common bls_sig_g1 bls_sig_g2 g1_common g2_common opt_sswu_g1 opt_sswu_g2
PYFILES := $(addprefix __sage__, $(addsuffix .py,$(SAGEFILES)))
.PRECIOUS: $(PYFILES)
.PHONY: pyfiles

__sage__%.py: %.sage
	@echo "Parsing $<"
	@sage --preparse $<
	@mv $<.py $@

pyfiles: $(PYFILES)

all: vec_g1 vec_g2

TEST_INPUTS := rfc6979
vec_%: bls_sig_%.sage pyfiles
	sage $< -T ../test-vectors/$(TEST_INPUTS) | tee $<.log

clean:
	rm -f __sage__*.py *.pyc *.sage.py
