
docs: handshake.dot
	cd coward_bot && cargo doc
	cd gameserver && cargo doc
	cd shotgun_common && cargo doc

handshake.png: handshake.dot
	dot -T png -o handshake.png handshake.dot
