#let config(
  title,
  candidate_name,
  body
) = {
  set document(title: title, author: candidate_name)
  set text(region: "GB")
  
  // Cover page
  page[
    #align(left + horizon)[
      #text(weight: "bold", size: 3em, title)
      
      #text(size: 1.5em, candidate_name)
    ]
  ]

  set page(
    numbering: "1",
    header: align(right, smallcaps(title)),
  )

  // Contents
  page(outline(depth: 3))

  set heading(numbering: "1.")
  set enum(full: true)

  show "Objective X": text(color.red, "Objective X", weight: "bold")

  body

  page(bibliography("bibliography.yml"))
}

#let question_counter = counter("question")

#let question(question, answer, analysis) = {
  question_counter.step()
  
  block(spacing: 2em, breakable: false)[
    *Question #{context question_counter.display()}:* #quote(question)

    #quote(block: true, quotes: true, answer)

    #analysis
  ]
}

#let pad_zero(number) = {
  if int(number) < 10 {
    "0" + number
  } else {
    number
  }
}

#let test_counter = counter("test")

#let test(objectives, description) = { 
  ( 
    context test_counter.display(),
    objectives,
    description,
    context raw("t" + pad_zero(test_counter.display()) + ".slang"),
    context raw("t" + pad_zero(test_counter.display()) + ".out") + test_counter.step(),
    [Pass]
  )
}

#let test_na(objectives, description) = { 
  ( 
    context test_counter.display(),
    objectives,
    description,
    context raw("t" + pad_zero(test_counter.display()) + ".slang") + [ (in `na` mode).],
    context raw("t" + pad_zero(test_counter.display()) + ".out") + test_counter.step(),
    [Pass]
  )
}

#let test_rc(objectives, description) = { 
  ( 
    context test_counter.display(),
    objectives,
    description,
    context raw("t" + pad_zero(test_counter.display()) + ".slang") + [ (in `rc` mode).],
    context raw("t" + pad_zero(test_counter.display()) + ".out") + test_counter.step(),
    [Pass]
  )
}


#let test_gc(objectives, description) = { 
  ( 
    context test_counter.display(),
    objectives,
    description,
    context raw("t" + pad_zero(test_counter.display()) + ".slang") + [ (in `gc` mode).],
    context raw("t" + pad_zero(test_counter.display()) + ".out") + test_counter.step(),
    [Pass]
  )
}


#let special_test(objectives, description, data, expected_outcome) = {
  ( 
    context test_counter.display() + test_counter.step(),
    objectives,
    description,
    data,
    expected_outcome,
    [Pass]
  )
}

#let appendix(body) = {
  counter(heading).update(0)
  set heading(numbering: "A.1", supplement: [Appendix])
  body
}
