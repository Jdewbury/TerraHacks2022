public class Board {
    private int[] board=new int[100];

    public Board(){
        for (int i=0;i<100;i++) {

        }
    }

    public int[] getBoard() {
        return board;
    }

    public int getValueAt(int location){
        return board[location];
    }

    public Boat getBoatAt(Boat[] boats,int location){
        if (location!=0){
            for (Boat boat:boats) {
                if (boat!=null&&boat.getLength()==board[location]){
                    return boat;
                }
            }
            System.out.println("no such boat");
        }
        System.out.println("no boat here");
        return null;
    }

    public void addBoat(Boat boat){
        for (int location:boat.getLocation()) {
            board[location]=boat.getLength();
        }
    }


}
