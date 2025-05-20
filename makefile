
build:
	docker compose up -d --build --force-recreate --remove-orphans

test:
	docker buildx build --platform linux/amd64 -t test -f dockerfile.test . 
	docker run -i --rm --platform linux/amd64 test