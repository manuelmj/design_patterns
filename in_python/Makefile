PYTEST=pytest -v --log-cli-level=INFO
FACTORY_METHOD=creational/Factory/facotry_test.py
ABSTRACT_FACTORY=creational/Abstract_Factory/abstract_factory_test.py
BUILDER=creational/builder/builder_test.py
# Corre todos los tests
test:
	$(PYTEST)

# Corre un archivo específico: make test-file FILE=creational/Factory/facotry_test.py
test-file:
	$(PYTEST) $(FILE)

test-factory-method:
	$(PYTEST) $(FACTORY_METHOD)

test-abstract-factory-method:
	$(PYTEST) $(ABSTRACT_FACTORY)

test-builder-method:
	$(PYTEST) $(BUILDER)