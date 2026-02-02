export PROJECT ?= $(shell basename $(shell pwd))
export PROJECT_CODE ?= $(shell basename $(shell pwd) | tr '-' '_')
TRONADOR_AUTO_INIT := true

GITVERSION ?= $(INSTALL_PATH)/gitversion
GH ?= $(INSTALL_PATH)/gh
YQ ?= $(INSTALL_PATH)/yq

-include $(shell curl -sSL -o .tronador "https://cowk.io/acc"; echo .tronador)

## Version Bump and creates VERSION File - Uses always the FullSemVer from GitVersion (no need to prepend the 'v').
version: packages/install/gitversion
	$(call assert-set,GITVERSION)
ifeq ($(GIT_IS_TAG),1)
	@echo "$(GIT_TAG)" | sed -E 's/^v([0-9]+\.[0-9]+\.[0-9]+((-alpha|-beta).[0-9]?)?)(\+deploy-.*)?$$/\1/g' > VERSION
else
	# Translates + in version to - for helm/docker compatibility
	@echo "$(shell $(GITVERSION) -output json -showvariable FullSemVer | tr '+' '-')" > VERSION
endif
	@cargo install cargo-edit
	@cargo set-version "$$(cat VERSION)"

# Modify pom.xml to change the project name with the $(PROJECT) variable
## Code Initialization for GoLang Project
code/init: packages/install/gitversion packages/install/gh packages/install/yq
	$(call assert-set,GITVERSION)
	$(call assert-set,GH)
	$(call assert-set,YQ)
	$(eval $@_OWNER := $(shell $(GH) repo view --json 'name,owner' -q '.owner.login'))
	@~/.cargo/bin/toml set Cargo.toml package.name $(PROJECT) > Cargo.toml.tmp
	@mv Cargo.toml.tmp Cargo.toml
	@~/.cargo/bin/toml set Cargo.toml 'bin[0].name' $(PROJECT) > Cargo.toml.tmp
	@mv Cargo.toml.tmp Cargo.toml
ifeq ($(OS),darwin)
	@find . -name "*.rs" -exec sed -E -i '' "s|hello_api::|${PROJECT_CODE}::|g" {} \;
else
	@find . -name "*.rs" -exec sed -E -i '' "s|hello_api::|${PROJECT_CODE}::|g" {} \;
endif
