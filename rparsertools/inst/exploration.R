ap <- available.packages()
devtools::document()
devtools::load_all()

system.time({
pkg_deps <-parse_package_dependencies(ap[, "Package"], ap = ap)
})

library(dplyr)

inbuilt <- c(installed.packages(priority = "base")[, "Package"], installed.packages(priority = "recommended")[, "Package"])

pkg_deps <- pkg_deps |> 
  mutate(inbuilt = dependency %in% c("R", inbuilt))

pkg_deps |> 
  count(inbuilt)


pkg_deps |> 
  count(package) |> 
  arrange(desc(n)) |> 
  slice(1:20)

pkg_deps |> 
  filter(type != "Suggests") |> 
  count(package) |> 
  arrange(desc(n)) |> 
  slice(1:20)

pkg_deps |> 
  filter(type != "Suggests", !inbuilt) |> 
  mutate(has_constraint = !is.na(constraint)) |>
  count(dependency, has_constraint) |> 
  group_by(dependency) |> 
  mutate(total = sum(n)) |>
  ungroup() |> 
  arrange(desc(total)) |> 
  slice(1:20)


# how do constraints evolve

pkg_deps |> 
  filter(dependency == "rlang") |> 
  mutate(has_constraint = !is.na(constraint)) |>
  count(type, has_constraint) |> 
  tidyr::pivot_wider(names_from = has_constraint, values_from = n)

pkg_deps |> 
  filter(dependency == "rlang", !is.na(constraint)) |> 
  count(constraint) |> arrange(desc(n))

pkg_deps |> 
  filter(dependency == "rlang", !is.na(constraint)) |> 
  count(constraint) |> arrange(desc(constraint))


pkg_deps |> 
  filter(!is.na(operator)) |> 
  count(operator)
