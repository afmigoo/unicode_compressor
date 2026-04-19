def get_printable_utf8(size: int):
    result = []

    for i in range(pow(2, 8 * size)):
        try:
            ch = chr(i)
        except ValueError:
            break
        if ch.isprintable() and not ch.isspace() and len(ch.encode('utf-8')) == size:
            result.append(ch)

    return result

if __name__ == "__main__":
    for i in range(1, 4):
        printable = get_printable_utf8(i)
        print(f'Number of {i}-byte printable characters:', len(printable))
