module Luhn (isValid) where

isValid :: String -> Bool
isValid n =
  let convertedNumbers  = convertToNumbers n
      numberedList      = zip (reverse convertedNumbers) [1..]
      doubleEveryOther  = map (\(x, y) -> if even y
                                          then x * 2
                                          else x) numberedList
      reduceTooBig      = map (\x -> if x > 9
                                     then x - 9
                                     else x) doubleEveryOther
  in  length convertedNumbers /= 1 && sum reduceTooBig `mod` 10 == 0

convertToNumbers :: String -> [Int]
convertToNumbers [] = []
convertToNumbers xs = map (read . (:"")) $ filter (/=' ') xs
