class UndergroundSystem(object):

    def __init__(self):
        self.checkIns = {}  # id -> (startStation, time)
        self.trips = {}     # (startStation, endStation) -> (count, timeTotal)

    def checkIn(self, id, stationName, t):
        self.checkIns[id] = (stationName, t)

    def checkOut(self, id, stationName, t):
        (startStation, start_t) = self.checkIns[id]

        key = (startStation, stationName)

        (count, timeTotal) = self.trips.getDefault(key, (0, 0))

        self.trips[key] = (count + 1, timeTotal + (t - start_t))

    def getAverageTime(self, startStation, endStation):
        self.trips.getDefault((startStation, endStation), (0, 0))
    
undergroundSystem = UndergroundSystem()
undergroundSystem.checkIn(45, "Leyton", 3)
undergroundSystem.checkIn(32, "Paradise", 8)
undergroundSystem.checkIn(27, "Leyton", 10)
undergroundSystem.checkOut(45, "Waterloo", 15)  # Customer 45 "Leyton" -> "Waterloo" in 15-3 = 12
undergroundSystem.checkOut(27, "Waterloo", 20)  # Customer 27 "Leyton" -> "Waterloo" in 20-10 = 10
undergroundSystem.checkOut(32, "Cambridge", 22) # Customer 32 "Paradise" -> "Cambridge" in 22-8 = 14
undergroundSystem.getAverageTime("Paradise", "Cambridge") # return 14.00000. One trip "Paradise" -> "Cambridge", (14) / 1 = 14
undergroundSystem.getAverageTime("Leyton", "Waterloo")    # return 11.00000. Two trips "Leyton" -> "Waterloo", (10 + 12) / 2 = 11
undergroundSystem.checkIn(10, "Leyton", 24)
undergroundSystem.getAverageTime("Leyton", "Waterloo")    # return 11.00000
undergroundSystem.checkOut(10, "Waterloo", 38)  # Customer 10 "Leyton" -> "Waterloo" in 38-24 = 14
undergroundSystem.getAverageTime("Leyton", "Waterloo")    # return 12.00000. Three trips "Leyton" -> "Waterloo", (10 + 12 + 14) / 3 = 12