import Data.Char
import Data.List
import Data.Function

binToInt :: String -> Int
binToInt bits = foldl (\acc c -> (digitToInt c) + 2 * acc) 0 $ bits

mostCommon :: Ord a => [a] -> a
mostCommon = head . maximumBy (compare `on` length) . group . sort

leastCommon :: Ord a => [a] -> a
leastCommon = head . minimumBy (compare `on` length) . group . sort

decode :: (String -> Char) -> [String] -> Int
decode f strs = binToInt $ go 0 max strs
  where
    max = length (transpose strs) - 1
    go pos maxPos xs = needle : if   pos < maxPos
                                then go (pos + 1) maxPos (filter keepNeedle xs)
                                else []
      where keepNeedle = (\str -> (str !! pos) == needle)
            needle = f $ (!!pos) (transpose xs)

oxygen :: [String] -> Int
oxygen = decode mostCommon

co2 :: [String] -> Int
co2 = decode leastCommon

solve :: [String] -> Int
solve nums = (oxygen nums) * (co2 nums)

main :: IO ()
main = interact $ show . solve . lines
