# UEBA solutions
Used to detect zero-day + insider threats on networks, that would likely evade the traditional security tools.

Creation of a baseline of "normal" behaviors, employing a combination of machine learning with statistical analysis, in order to identify deviations from those baselines .

# Tools
- ActivTrak

- Cynet - analises behaviour based on role, group, geolocation, working hours to define normal patterns. Then it monitors user activity in real time. 

- QRadar 
    - 2 main functions
        - Risk Profiling - assign risk level to various security use cases, using existing event and flow data. Assesses the severity and reliability of detected incidents.  
        -  Unified user identities - combines different user accounts associated to a single user, granting a  comprehensive consolidation of risk and traffic data.

- LogPoint - uses maching learning to detect anomalous behaviour. 

- Splunk User Behaviour Analytics - condenses billion of raw events into a manageable number of threats, allowing for efficient review. Allows *user feedback*

-Teramind 

# UEBA Components
- Data analytics in order to build a profile for each user, and keep a continuously monitoring of the behaviour

- Data integration from a wide range of sources, including logs, packet capture data, and information fed from other tools

- A way to present its findings to the users. Can be an alert that recommends further investigation, or an automatica atction to remediate the threat. 

# Papers
- https://ieeexplore.ieee.org/abstract/document/9614848?casa_token=nEWqXE2pLMsAAAAA:69QwJosGrSDeHYs8nqJHpfHcOl9iSzKmjw0-_5GODiGaJ_vsgvmErnAH63ueVHoz_EWb3F1B
- https://ieeexplore.ieee.org/abstract/document/10007245
- https://turcomat.org/index.php/turkbilmat/article/view/14394
- https://link.springer.com/chapter/10.1007/978-981-19-8493-8_32

- Malicious activity detection  An analysis of current tools and methodologies for network defence in operational networks
    - Link: https://cradpdf.drdc-rddc.gc.ca/PDFS/unc361/p813212_A1b.pdf

- https://dl.acm.org/doi/pdf/10.1145/3220199.3220221 -> seems appropriate

# Conclusions about tools and methodologies from the papers

## Data collection
Military solutions utilizes data collection in multiple locations.
 - Sensor placement on entry and exit of backbone, since all traffic passes through that sensor.
 - At the entry of each geographically different location.
We can collect traffic packets, even though that compression may be required due to large size of pcap files.
We can collect also flow data and metadata.
Log data should be captured at routers, servers, switches and hosts.

## Storage 
Usually a data mesh/fata lake. The distributed repository allows for the connection of distributed data across diferent locations ensuring high data availability and greater autonomy, which allows fexible experimentation and analytics

## Monitoring tools and methodologies for malicious activity detection 
Computer networks need monitoring in order to detect and identify malicious activities.
Network monitoring can be performed in a number of ways:
 - Packet Sniffers - most common and effective. Used to examine communication data streams in the network

## State of the art in network monitoring 

- Proactive network monitoring solution using deep learning. Convolutional neural network (CNN), gated recurrent unit (GRU), long-short term memory (LSTM), etc. for efective monitoring. This has been validated on both proprietary and publicly available datasets.

- Large scale network traffic monitoring system .Netflow for collecting real-time data, Logstash for transferring data, ElasticSearch for storing, and Kibana for displaying real-time analysis. The proposed system can achieve millisecond responses to 100 million of Netflows, which has the potential of meeting the need of large-scale network traffic.

### COTS tools 
- SentryWire - full packet lossless capture at speeds ranging from 1Mbps to 1 Tbps
- PacketScan -  packet capturing tool that is said to monitor networks and be able to capture packets in networks up to 10 Gps

## State of the Art in intrusion detection
Intrusion detection involves the collection of network activity through traffic monitoring or packet logging, followed by an analysis phase that identifies which of the activities could be malicious

### COTS tools 
- Snort is an open source IDS tool that can be used as an intrusion prevention system (IPS). It analyses real-time or historical trafc patterns and compares them against previously written signatures in the form of confguration fles

- Suricata  is a free open-source signature-based real time NIDS and inline IPS that also performs network security monitoring (NSM). It can perform analysis of pcap fles to support forensic investigations. Suricata supports standard input and output
formats like YAML4 and JSON. It can also be integrated with other tools such as security information and event management (SIEM) systems, Splunk, Kibana, etc.

### State of the Art Methodologies 
- Prediction of anomalies based on historical traffic using (GAMPAL) -  General-purpose Anomaly detection Mechanism using Path Aggregate without Labeled data General-purpose Anomaly detection Mechanism using Path Aggregate without Labeled data - 

- a flow-based method for anomalous traffic detection on large-scale backbone networks. The proposed method uses entropy and principal
component analysis (PCA) operators calculated from four fow-based traffic features. Check: Duarte Jr, E. P., Hara, C., Torres Jr, P., and Gomes, C. (2020), BackStreamDB: A stream processing engine for backbone trafc monitoring with anomaly detection, Security and Privacy, 3(3), e106.

- flow based IDS named BigFLow for high-speed networks. It can extract up to 158 statistical features, which have been used by a stream learning stage engine that classifes normal and attack trafc. BigFlow, which has been evaluated on MAWIFlow dataset, can support 10 GBps network bandwidth in a  40-core cluster commodity hardware. Check: Viegas, E., Santin, A., Bessani, A., and Neves, N. (2019), BigFlow: Real-time and
reliable anomaly-based intrusion detection for high-speed networks, Future
Generation Computer Systems, 93, pp. 473–485.



**Try to understand with what data collection we will be working with** 

**Network monitoring vs host monitoring - Which one will we be working with**








