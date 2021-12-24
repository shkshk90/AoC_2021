val = 3
for i in range(97, 123):
  char = chr(i)
  print(f"'{char}'\t=>\t{val},")
  val += 1
val = 33
for i in range(65, 91):
  char = chr(i)
  print(f"'{char}'\t=>\t{val},")
  val += 1
print('===')
val = 3
for i in range(97, 123):
  char = chr(i)
  print(f"{val}\t=>\t'{char}',")
  val += 1
val = 33
for i in range(65, 91):
  char = chr(i)
  print(f"{val}\t=>\t'{char}',")
  val += 1