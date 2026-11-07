#import "config.typ": config, appendix

#show: config.with(
  "Investigating memory management with a custom interpreter",
  "Sam Cutter"
)

#include "sections/analysis.typ"

#include "sections/design.typ"

#include "sections/technical_solution.typ"

#include "sections/testing.typ"

#include "sections/evaluation.typ"

#show: appendix

#include "appendices/prototype_code.typ"

#include "appendices/interpreter_code.typ"

#include "appendices/tests.typ"
