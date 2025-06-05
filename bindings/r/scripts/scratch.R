library(codelist)

c <- Codelist$new('test')
c$set_name('new_test')
e <- c$get_entries()

first <- e[[1]]$code

