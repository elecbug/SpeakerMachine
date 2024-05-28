namespace Packet
{
    public class Request
    {
        public Request(Request request)
        {
            Title = request.Title;
            Description = request.Description;
            Name = request.Name;
            RR = request.RR;
        }

        public Request() { }

        public string Title { get; set; } = "";
        public string Description { get; set; } = "";
        public string Name { get; set; } = "";
        public int RR { get; set; } = 0;
        public string DateTime { get; set; } = System.DateTime.Now.ToString("yyyy.MM.dd. HH:mm:ss");
    }
}
