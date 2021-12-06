%: %.rxe
	./$@.rxe

%/1.rs:
	mkdir -p $*
	cp template.rs $@

%/2.rs: | %/1.rs
	cp $| $@

.SECONDEXPANSION:
.SECONDARY:
%.rxe: %.rs $$(@D)/input
	rustc -o $@ $<

%/input: .cookie
	mkdir -p $$(dirname $@)
	curl https://adventofcode.com/2021/day/$*/input -H"Cookie: $$(cat .cookie)" > $@

clean:
	rm -rf *.rxe */*.rxe */input

all: $(foreach day,$(shell seq 1 24),$(foreach part,1 2,$(day)/$(part))) 25/1
