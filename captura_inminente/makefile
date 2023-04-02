all-1: build run
build:
	@echo "building app"
	@cargo build
casoE:
	@echo echo "Corriendo el caso de prueba empate"
	@cargo run -- tests/test_cases/caso-de-prueba-empate.txt

casoP:
	@echo "Corriendo el caso de prueba no gana ninguna"
	@cargo run -- tests/test_cases/caso-de-prueba-ninguna-gana.txt

casoB:
	@echo "Corriendo el caso de prueba ganan las blacnas"
	@cargo run -- tests/test_cases/caso_gana_blanca.txt

casoN:
	@echo "Corriendo el caso de prueba ganan las negras"
	@cargo run -- tests/test_cases/caso_gana_negra.txt