solve :: [Int] -> Int
solve ns = length $ filter (<0) $ zipWith (-) ns (tail ns)

convert :: [String] -> [Int]
convert = map read

main :: IO ()
main = interact $ show . solve . convert . lines
