
FORMAT = cargo fmt -- --check
CLIPPY = cargo clippy -- -D warnings
TEST = cargo test 
TEST_VERBOSE =-- --nocapture

#tests names 
FACTORY_METHOD=factory_method_tests
ABSTRACT_FACTORY_METHOD=abstract_factory_tests


# Corre todos los tests
test:
	$(PYTEST)


format-check:
	$(FORMAT)
	$(CLIPPY)
# Corre un archivo específico: make test-file FILE=creational/Factory/facotry_test.py
test-file:
	$(PYTEST) $(FILE)

test-factory-method:
	$(TEST) $(FACTORY_METHOD) $(TEST_VERBOSE)

test-abstract-factory-method:
	$(TEST) $(ABSTRACT_FACTORY_METHOD) $(TEST_VERBOSE)
