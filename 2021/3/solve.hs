import Data.Char
import Data.List
import Data.Function

binToInt :: String -> Int
binToInt bits = foldl (\acc c -> (digitToInt c) + 2 * acc) 0 $ bits

mostCommon :: Ord a => [a] -> a
mostCommon = head . maximumBy (compare `on` length) . group . sort

leastCommon :: Ord a => [a] -> a
leastCommon = head . minimumBy (compare `on` length) . group . sort

oxygen :: [String] -> Int
oxygen strs = binToInt $ go 0 ((length (transpose strs)) - 1) strs
  where
    go pos maxPos xs = top : if   pos < maxPos
                             then go (pos + 1) maxPos (filter (\str -> (str !! pos) == top) xs)
                             else []
      where top = mostCommon $ (!!pos) (transpose xs)

co2 :: [String] -> Int
co2 strs = binToInt $ go 0 ((length (transpose strs)) - 1) strs
  where
    go pos maxPos xs = bottom : if   pos < maxPos
                             then go (pos + 1) maxPos (filter (\str -> (str !! pos) == bottom) xs)
                             else []
      where bottom = leastCommon $ (!!pos) (transpose xs)

solve :: [String] -> Int
solve nums = (oxygen nums) * (co2 nums)

main :: IO ()
main = interact $ show . solve . lines
