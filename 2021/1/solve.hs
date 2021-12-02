addEachTriple :: [Int] -> [Int]
addEachTriple ns = zipWith3 (\a b c -> a + b + c) ns (tail ns) (tail . tail $ ns)

solve :: [Int] -> Int
solve ns = length $ filter (<0) $ zipWith (-) ns (tail ns)

convert :: [String] -> [Int]
convert = map read

main :: IO ()
main = interact $ show . solve . addEachTriple . convert . lines
