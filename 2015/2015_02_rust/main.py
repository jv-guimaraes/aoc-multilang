def day2_1():
    total = 0
    for line in open('input.txt'):
        l, w, h = line.split('x')
        l, w, h = int(l), int(w), int(h)
        area = 2*l*w + 2*w*h + 2*h*l
        slack = min(l*w, w*h, h*l)
        total += area + slack
    print(total)

def day2_2():
    total = 0
    for line in open('input.txt'):
        l, w, h = line.split('x')
        l, w, h = int(l), int(w), int(h)
        ribbon = 2 * min(l+w, w+h, h+l)
        bow = l*w*h
        total += ribbon + bow
    print(total)

if __name__ == '__main__':
    day2_1()
    day2_2()