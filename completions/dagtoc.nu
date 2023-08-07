module completions {

  export extern dagtoc [
    --delete(-d)              # Delete contents
    --add(-a): string         # Add contents via KDL
    --get(-g)                 # Get contents
    --check(-c)               # Check whether the contents is valid (all outlines numbered and pages are monotone increasing)
    --offset(-x): string      # ±count to pages of input/output TOC
    --output(-o): string      # Output PDF path
    pdf: string               # Input PDF path
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

}

use completions *
