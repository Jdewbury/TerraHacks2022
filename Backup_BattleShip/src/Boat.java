public class Boat {
    private int length;
    private int[] location;
    boolean alive=true;
    int countDMG=0;
    public Boat(int length){
        this.length=length;
        location=new int[length];
    }

    public void setLocation(int[] location) {
        for (int i = 0; i < location.length; i++) {
            this.location[i]=location[i];
        }
    }

    public int getLength() {
        return length;
    }

    public int[] getLocation() {
        return location;
    }

    public void setDamage(int location){
        countDMG++;
        System.out.println("hi"+countDMG);
        if (countDMG==length){
            alive=false;
        }
    }
}
