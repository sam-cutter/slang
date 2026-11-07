#show: page

#import "../config.typ" : pad_zero

= Tests <tests>

#set heading(outlined: false)

#for test in range(45) [
  == #raw("t" + pad_zero(str(test)) + ".slang")
  #raw(read("../source_code/interpreter/tests/t" + pad_zero(str(test)) + ".slang"), lang: "")

  == #raw("t" + pad_zero(str(test)) + ".out")
  #raw(read("../source_code/interpreter/tests/t" + pad_zero(str(test)) + ".out"), lang: "")
]

== `t45.slang`
#raw(read("../source_code/interpreter/tests/t45.slang"), lang: "")

== `t52.slang`
#raw(read("../source_code/interpreter/tests/t52.slang"), lang: "")