build:
	rm -fr out/
	soroban contract build --out-dir out
	for o in out/*.wasm ; do soroban contract optimize --wasm $$o; done
	ls -lah out/
