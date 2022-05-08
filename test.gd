extends Node

var gifdot_lib = preload("res://addons/gifdot/gifdot.gdns")

func _ready():
	var black = Image.new()
	black.create(32, 32, false, Image.FORMAT_RGBA8)
	black.fill(Color.red)
	var white = Image.new()
	white.create(32, 32, false, Image.FORMAT_RGBA8)
	white.fill(Color.white)
	var enc = Gifdot.get_encoder(32, 32)
	enc.add_frame(black.get_data(), 10)
	enc.add_frame(white.get_data(), 10)
	var data = enc.get_file_data()
	var file = File.new()
	file.open("res://test.gif", File.WRITE)
	file.store_buffer(data)
	file.close()
