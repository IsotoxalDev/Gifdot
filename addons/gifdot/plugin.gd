tool
extends EditorPlugin

var gifdot_lib = preload("res://addons/gifdot/gifdot.gdns")

func _enter_tree():
	var gifdot = gifdot_lib.new()
	add_autoload_singleton("Gifdot", "res://addons/gifdot/gifdot.gdns")

func _exit_tree():
	remove_autoload_singleton("Gifdot")
