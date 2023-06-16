performance = debug
# performance = release 

# common commands
ifeq ($(shell echo ^^),^)
  	# cmd
  	delete  = $(info Deleting $1 ...) @del /q $(subst /,\,$1)
  	rdelete = $(info Deleting $1 ...) @del /s /q $(subst /,\,$1) && if exist $(subst /,\,$1) (rmdir /s /q $(subst /,\,$1))
	copy = $(info Copying $1 to $2 ...) @copy /Y $(subst /,\,$1) $(subst /,\,$2)
else
  	# sh
  	delete  = $(info Deleting $1 ...) @rm -f $1
  	rdelete = $(info Deleting $1 ...) @rm -rf $1
	copy = $(info Copying $1 to $2 ...) @cp $1 $2
endif

all: build

build:
	cmake -GNinja -Bcmake-build-$(performance) -DCMAKE_EXPORT_COMPILE_COMMANDS=ON
	$(call copy,./cmake-build-$(performance)/compile_commands.json,.)
run:
	ninja -C cmake-build-$(performance)
	./cmake-build-$(performance)/ngimg



