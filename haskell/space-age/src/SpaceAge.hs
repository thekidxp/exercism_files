module SpaceAge (Planet(..), ageOn) where

data Planet = Mercury
            | Venus
            | Earth
            | Mars
            | Jupiter
            | Saturn
            | Uranus
            | Neptune

ageOn :: Planet -> Float -> Float
ageOn planet seconds = case planet of Mercury -> earthTime / 0.2408467
                                      Venus   -> earthTime / 0.61519726
                                      Earth   -> earthTime
                                      Mars    -> earthTime / 1.8808158
                                      Jupiter -> earthTime / 11.862615
                                      Saturn  -> earthTime / 29.447498
                                      Uranus  -> earthTime / 84.016846
                                      Neptune -> earthTime / 164.79132
                                      where earthTime = seconds / 31557600
