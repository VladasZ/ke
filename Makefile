lint:
	cargo clippy \
      -- \
      \
      -W clippy::all \
      -W clippy::pedantic \
      \
      -A clippy::missing_panics_doc \
      -A clippy::module_name_repetitions \
      -A clippy::missing_errors_doc \
      -A clippy::must_use_candidate \
      -A clippy::module_inception \
      -A clippy::needless_pass_by_value \
      \
      -D warnings
