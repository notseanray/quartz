struct DistrictRankingsResponse {
    districtRanks: Vec<Team>,
}

/*
 *     {
      "districtCode": "PCH",
      "teamNumber": 2974,
      "rank": 1,
      "totalPoints": 149,
      "event1Code": "GADAL",
      "event1Points": 73,
      "event2Code": "GACAR",
      "event2Points": 76,
      "districtCmpCode": null,
      "districtCmpPoints": null,
      "teamAgePoints": 0,
      "adjustmentPoints": 0,
      "qualifiedDistrictCmp": false,
      "qualifiedFirstCmp": false
    },

 */

#[allow(non_snake_case)]
struct Team {
    districtCode: Option<String>,
    teamNumber: Option<usize>,
    rank: Option<usize>,
    totalPoints: Option<usize>,
    event1Code: Option<String>,
    event2Points: Option<usize>,
    districtCmpCode: Option<String>,
    districtCmpPoints: Option<usize>,
    teamAgePoints: Option<usize>,
    adjustmentPoint: Option<usize>,
    qualifiedDistrictCmp: Option<bool>,
    qualifiedFirstCmp: Option<bool>,
}

#[allow(non_snake_case)]
struct UnwrapedTeam {
    districtCode: String,
    teamNumber: usize,
    rank: usize,
    totalPoints: usize,
    event1Code: String,
    event2Points: usize,
    districtCmpCode: String,
    districtCmpPoints: usize,
    teamAgePoints: usize,
    adjustmentPoint: usize,
    qualifiedDistrictCmp: bool,
    qualifiedFirstCmp: bool,
}

impl Team {
    pub fn default_values(t: Team) -> UnwrapedTeam {
        UnwrapedTeam {
            districtCode: t.districtCode.unwrap_or_default(),
            teamNumber: t.teamNumber.unwrap_or_default(),
            rank: t.rank.unwrap_or_default(),
            totalPoints: t.totalPoints.unwrap_or_default(),
            event1Code: t.event1Code.unwrap_or_default(),
            event2Points: t.event2Points.unwrap_or_default(),
            districtCmpCode: t.districtCmpCode.unwrap_or_default(),
            districtCmpPoints: t.districtCmpPoints.unwrap_or_default(),
            teamAgePoints: t.teamAgePoints.unwrap_or_default(),
            adjustmentPoint: t.adjustmentPoint.unwrap_or_default(),
            qualifiedDistrictCmp: t.qualifiedDistrictCmp.unwrap_or_default(),
            qualifiedFirstCmp: t.qualifiedFirstCmp.unwrap_or_default(),
        }
    }
}
