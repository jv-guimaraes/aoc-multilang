RANGE = range(271973, 785961)

def is_valid(password: int) -> bool:
    bytes = str(password)

    for i in range(len(bytes)-1):
        if bytes[i] > bytes[i+1]: return False
    
    for i in range(len(bytes)-1):
        if bytes[i] == bytes[i+1]: return True

    return False

def is_valid2(password: int) -> bool:
    b = str(password)

    for i in range(len(b)-1):
        if b[i] > b[i+1]: return False

    if b[0] == b[1] and b[1] != b[2]: return True
    if b[1] == b[2] and b[1] != b[0] and b[1] != b[3]: return True
    if b[2] == b[3] and b[2] != b[1] and b[2] != b[4]: return True
    if b[3] == b[4] and b[3] != b[2] and b[3] != b[5]: return True
    if b[4] == b[5] and b[4] != b[3]: return True

    return False


counter = 0
for i in RANGE:
    if is_valid(i): counter += 1
print(f"Parte 1: {counter}")

counter = 0
for i in RANGE:
    if is_valid2(i): counter += 1
print(f"Parte 2: {counter}")