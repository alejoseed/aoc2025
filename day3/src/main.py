def main():
    cases = [""]
    total = 0

    for test in cases:
        stack = []
        max_length = 12

        idx = 0
        cur = 12
        while len(stack) != max_length:
            dummy = "0"
            stack.append(dummy)
            cur_idx = idx
            final_idx = 0

            while cur <= len(test) - cur_idx:
                head = stack.pop()
                if int(head) < int(test[cur_idx]):
                    stack.append(test[cur_idx])
                    final_idx = cur_idx
                else:
                    stack.append(head)
                cur_idx += 1
            cur -= 1
            idx = final_idx + 1
        print(''.join(stack))
        total += int(''.join(stack))
    print(total)
main()
