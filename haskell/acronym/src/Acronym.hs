module Acronym where
import Data.Char

abbreviate :: String -> String
abbreviate xs =
    let justCapitals = filter isUpper . capitalize
        splitWords ys = concatMap (splitOn '-') $ words ys
    in  concatMap justCapitals $ splitWords xs

capitalize :: String -> String
capitalize [] = ""
capitalize (x:xs) = if isAllCaps (x:xs)
  then toUpper x : map toLower xs
  else toUpper x : xs

isAllCaps :: String -> Bool
isAllCaps [] =  False
isAllCaps [x] = isUpper x
isAllCaps (x:xs) = isUpper x && isAllCaps xs

splitOn :: Char -> String -> [String]
splitOn c s = case dropWhile (c ==) s of
                "" -> []
                s' -> w : splitOn c s''
                      where (w, s'') = break (c ==) s'
