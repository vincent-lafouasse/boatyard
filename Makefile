alpine_c_cpp: DOCKERFILE = Dockerfile.c_cpp
alpine_c_cpp:
	docker build --file $(DOCKERFILE) -t $@ .

.PHONY: alpine_c_cpp
