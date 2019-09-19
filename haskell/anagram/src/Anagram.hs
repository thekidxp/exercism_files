module Anagram (anagramsFor) where
import Data.Char

anagramsFor :: String -> [String] -> [String]
anagramsFor xs xss = let  getAnagrams       = filter (isAnagram xs) xss
                          isUnique ys zs = map toLower ys /= map toLower zs
                     in   filter (isUnique xs) getAnagrams

isAnagram :: String -> String -> Bool
isAnagram [] []          = True
isAnagram (_:_) []       = False
isAnagram [] (_:_)       = False
isAnagram [x] [y]        = toLower x == toLower y
isAnagram (_:_) [_]      = False
isAnagram [_] (_:_)      = False
isAnagram (char:rest) xs = let char' = toLower char
                               rest' = map toLower rest
                               xs'   = map toLower xs
                           in  case dropWhile (/= char') xs' of
                               ""   ->  False
                               _    ->  isAnagram rest' xs''
                                        where xs'' = first ++ second
                                              (first, _:second) = break (== char') xs'
