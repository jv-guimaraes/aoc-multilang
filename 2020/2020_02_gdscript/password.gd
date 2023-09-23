var lo; var hi; var c; var text

func _init(_lo, _hi, _c, _text):
    lo = _lo
    hi = _hi
    c = _c
    text = _text

func _to_string():
    return "{%d, %d, %c, %s}" % [lo, hi, c, text]

func is_valid() -> bool:
    var count = 0
    for letter in text:
        if letter == c:
            count += 1
    return count >= lo and count <= hi

func is_valid2() -> bool:
    var pos1 = text[lo - 1] == c
    var pos2 = text[hi - 1] == c
    return (pos1 and not pos2) or (pos2 and not pos1)