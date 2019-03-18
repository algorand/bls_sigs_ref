# makefile for hash-to-curve PoC code
#
# (C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

TEST_PROGS := hash_to_base opt_sswu_g1 opt_sswu_g2 svdw_g1 svdw_g2

all: test

define MAKE_TEST
test_$(1): hash_to_base.py
	sage $(1).sage
endef
$(foreach test,$(TEST_PROGS),$(eval $(call MAKE_TEST,$(test))))

%.py: %.sage
	sage --preparse $<
	mv $<.py $@

test: $(addprefix test_,$(TEST_PROGS))

clean:
	rm -f *.pyc *.sage.py hash_to_base.py
