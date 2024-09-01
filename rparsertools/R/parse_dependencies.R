#' parse package dependencies
#' @param packages vector of package names
#' @param ap dataframe-like structure containing dep vectors returned by available.packages()
#' @param dep_types vector of dependency types such as Depends, Imports, etc
#' @param filter_missing filter packages that have no dependencies 
#' @param .df whether to return a data frame, default: TRUE
#' @details
#' available.packages returns the dependency of a package as a single string that is comma separated
#' and unparsed containing things like newlines.
#' @export
parse_package_dependencies <- function(packages, ap, dep_types = c("Depends", "Imports", "Suggests", "LinkingTo"), filter_missing = TRUE, .df = TRUE) {
  # TODO: add error checking for dep_types being valid columns
  ap_df <- as.data.frame(ap[, c("Package", dep_types)], stringsAsFactors = FALSE)
  res <- lapply(dep_types, function(.dt) {
    out <- parse_all_package_dependencies_impl(packages, ap_df[[.dt]], TRUE)
    out$type <- .dt
    return(out)
  })
  if (!.df) {
    return(res)
  }
  return(do.call(rbind, res))
} 