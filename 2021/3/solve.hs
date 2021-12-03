import Data.Char
import Data.List
import Data.Function

binToInt :: String -> Int
binToInt bits = foldl (\acc c -> (digitToInt c) + 2 * acc) 0 $ bits

mostCommon :: Ord a => [a] -> a
mostCommon = head . maximumBy (compare `on` length) . group . sort

leastCommon :: Ord a => [a] -> a
leastCommon = head . minimumBy (compare `on` length) . group . sort

solve :: [String] -> Int
solve nums = epsilon * gamma
  where epsilon = binToInt . map mostCommon . transpose $ nums
        gamma = binToInt . map leastCommon . transpose $ nums

main :: IO ()
main = interact $ show . solve . lines
