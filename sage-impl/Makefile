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

vec_%: bls_sig_%.sage pyfiles
	sage $< -d | tee $<.log

clean:
	rm __sage__*.py *.pyc *.sage.py
