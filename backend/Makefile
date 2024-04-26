.PHONY: all
all:
	@echo "+ $@"
	@rustup default stable
	@cross build --release
	@mkdir -p ../bin
	@cp ./target/x86_64-unknown-linux-gnu/release/tomoon ../bin/tomoon


.PHONY: clean 
clean:
	@echo "+ $@"
	@rm -rf ./target
	@rm -rf ../bin/tomoon