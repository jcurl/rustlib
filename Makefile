LIBS := readelf

all: .PHONY $(LIBS)
	@for subdir in $(LIBS); do \
	  $(MAKE) -C $$subdir all; \
	done

release: .PHONY $(LIBS)
	@for subdir in $(LIBS); do \
	  $(MAKE) -C $$subdir release; \
	done

clean: .PHONY $(LIBS)
	@for subdir in $(LIBS); do \
	  $(MAKE) -C $$subdir clean; \
	done

.PHONY:
